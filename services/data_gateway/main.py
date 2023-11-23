import datetime
import os
import threading

from dotenv import load_dotenv

import mysql.connector
from mysql.connector.pooling import PooledMySQLConnection
from mysql.connector.cursor import MySQLCursor

import paho.mqtt.client as mqtt
from paho.mqtt.client import Client as MqttClient

load_dotenv()


from azure.iot.device import IoTHubDeviceClient
from enum import Enum
from json import dumps

# device client errors enum
class DeviceClientErrors(Enum):
    FAILED_TO_CONNECT = 1
    FAILED_TO_SEND_TELEMETRY = 2
    FAILED_TO_PATCH_TWIN = 3

class DeviceClient: 
    device_client: IoTHubDeviceClient

    def __init__(self, connection_str: str, property_patched) -> None:
        if self.connect(connection_str):
            return

        self.device_client.receive_twin_desired_properties_patch = property_patched

    def send_telemetry(self, telemetry: dict) -> None:
        json = dumps(telemetry)
        try: 
            self.device_client.send_message(json)
        except:
            return DeviceClientErrors.FAILED_TO_SEND_TELEMETRY

    def patch_twin(self, update: dict) -> None:
        try: 
            self.device_client.patch_twin(update)
        except:
            return DeviceClientErrors.FAILED_TO_PATCH_TWIN

    def connect(self, connection_str: str) -> None | DeviceClientErrors:
        try: 
            self.device_client = IoTHubDeviceClient.create_from_connection_string(connection_str)
        except Exception as e:
            print(e)
            return DeviceClientErrors.FAILED_TO_CONNECT


# global variables
db: PooledMySQLConnection
cursor: MySQLCursor
device_client: DeviceClient
mqtt_client: MqttClient


def on_connect(client, userdate, flags, rc):
    print("Connected with result code " + str(rc))
    client.subscribe(os.getenv("MQTT_TOPIC") + '#')


def store_in_database(row: dict):
    query = "INSERT INTO measurements (type, value, send) VALUES (%s, %s, %s)"
    val = (row['topic'], row['value'], row['send'])

    cursor.execute(query, val)
    db.commit()


import time


def upload_old_data():
    while True:
        uploaded_ids = []
        try:
            for t in ['TEMPERATURE', 'HUMIDITY', 'PRESSURE']:
                # retrieve from db
                query = f"SELECT id, type, value, timestamp FROM measurements WHERE send = 0 AND type = '{t}' limit 100"
                cursor.execute(query),
                rows = cursor.fetchall()

                # if there is nothing to update don't do anything
                if len(rows) == 0:
                    continue

                # send [:-1] as telemetry
                for row in rows[:-1]:
                    device_client.send_telemetry({
                        row[1]: row[2],
                        'timestamp': row[3].strftime('%Y-%m-%d %H:%M:%S')
                    })
                    uploaded_ids.append((row[0],));

                query = f"select id from measurements where send = 1 AND type = '{t}' ORDER BY timestamp desc LIMIT 1"
                cursor.execute(query)
                latest_row = cursor.fetchone()

                if latest_row[0] < rows[-1][0]:
                    device_client.patch_twin({
                        rows[-1][1]: rows[-1][2],
                        'updated': rows[-1][1]
                    })
                else:
                    device_client.send_telemetry({
                        rows[-1][1]: rows[-1][2],
                        'timestamp': rows[-1][3].strftime('%Y-%m-%d %H:%M:%S')
                    })
                uploaded_ids.append((rows[-1][0], ))

        except:
            print('something went wrong')

        # update all uplaoded ids
        print(f'send {len(uploaded_ids)} messages to azure')
        query = 'UPDATE measurements SET send=1 WHERE id = %s'
        cursor.executemany(query, uploaded_ids)
        db.commit()

        time.sleep(10 * 60)  # wait 10 minutes to run upload the data


def on_message(client, userdata, msg):
    # parse topic and payload
    value = msg.payload.decode("utf-8")
    topic = (msg.topic.split('/')[-1])

    send = 0
    stored = 0

    # update the digital twin. only if it is something we want in the digital twin
    if topic in ['temperature', 'pressure', 'humidity', 'delay']:
        send = 1 if device_client.patch_twin({
            topic: value,
            "updated": topic
        }) == None else 0

        # check if we want to store this topic
        if topic in ['temperature', 'pressure', 'humidity']:
            store_in_database({
                'send': send,
                'topic': topic,
                'value': value
            })

            stored = 1

    print(f"Received message: {{\"{topic}\": \"{value}\"}}. Send to azure: {send}, stored in db: {stored}")


def property_patched(update: dict):
    for key, value in update.items():
        print(key, value)

        if key == 'delay':
            delayTopic = os.getenv('MQTT_TOPIC') + 'delay'
            mqtt_client.publish(delayTopic, value)


def main():
    global device_client, db, cursor, mqtt_client

    # make all connections
    # setup digital twin
    device_client = DeviceClient(
        os.getenv("AZURE_CONNECTION_STRING"),
        property_patched
    )

    db = mysql.connector.connect(
        host=os.getenv("MYSQL_HOST") or "database",
        user=os.getenv("MYSQL_USER") or "root",
        password=os.getenv("MYSQL_PASS"),
        database=os.getenv("MYSQL_DATABASE") or "data",
    )
    cursor = db.cursor()

    # setup mqtt connection
    mqtt_client = mqtt.Client()
    
    mqtt_client.on_connect = on_connect
    mqtt_client.on_message = on_message

    # connect to mqtt broker
    mqtt_client.username_pw_set(
        os.getenv("MQTT_USER"),
        os.getenv("MQTT_PASSWORD"))
    mqtt_client.connect(
        os.getenv("MQTT_HOST"),
        1883)

    # start upload loop
    t1 = threading.Thread(target=upload_old_data)
    t1.daemon = True
    t1.start()

    mqtt_client.loop_forever()


if __name__ == "__main__":
    main()
