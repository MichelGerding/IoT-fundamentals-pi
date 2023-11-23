const tempElem = document.querySelector('.temp .val');
const humidElem = document.querySelector('.humidity .value');
const pressElem = document.querySelector('.pressure .value');
const expectationElem = document.querySelector('.currently .value');

const image = document.querySelector('.report img');

// get the current hour
const date = new Date();
const hour = date.getHours();


fetch(`/api/latest/${hour}`)
    .then(res => res.json())
    .then(measurements => {
        console.log(measurements)
        tempElem.innerHTML = Math.round(measurements.temperature)
        pressElem.innerHTML = measurements.pressure.toFixed(2)
        humidElem.innerHTML = measurements.humidity.toFixed(2)
        expectationElem.innerHTML = measurements.rain_prediction == 1 ? 'Rain' : 'No rain';
        image.src = measurements.rain_prediction == 1 ? '/static/images/rain.png' : '/static/images/sun.png';
    })