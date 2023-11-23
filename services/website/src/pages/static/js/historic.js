// import uPlot from '../uplot/main.mjs';
//
//
// function genGraphData(dataIn) {
//     let minVal = 99999;
//     let maxVal = -9999999;
//
//     let minDate;
//     let maxDate;
//
//
//     let dates = [];
//
//     dataIn.sort((a,b) => {
//         const at = new Date(a.timestamp);
//         const bt = new Date(b.timestamp);
//
//         if (at.getTime() < bt.getTime()) {
//             return -1
//         } else if (at.getTime() > bt.getTime()) {
//             return 1
//         }
//
//         return 0;
//     })
//
//     let dataPoints = dataIn.map(temp => {
//         const dt = new Date(temp.timestamp);
//         dates.push(+dt / 1000)
//
//         if (!minDate || minDate.getTime() > dt.getTime()) {
//             minDate = dt
//         }
//
//         if (!maxDate || maxDate.getTime() < dt.getTime()) {
//             maxDate = dt
//         }
//
//         minVal = Math.min(minVal, temp.value)
//         maxVal = Math.max(maxVal, temp.value)
//
//         return temp.value;
//     });
//
//     let dataset = [
//        dates,
//         dataPoints
//     ]
//
//     return {
//         dataset,
//         minDate,
//         maxDate,
//         minVal,
//         maxVal
//     }
// }
//
// let tempChart;
// let pressChart;
// let humidChart;
//
//
// export function main() {
//     console.log(tempChart)
//
//     window.addEventListener("resize", e => {
//         u.setSize(getSize());
//     });
//
//     fetch('/api/temps')
//         .then(res => res.json())
//         .then((temps) => {
//
//
//             const {dataset, minVal, maxVal, minDate, maxDate} = genGraphData(temps)
//
//             let options = {
//                 width: 1920,
//                 height: 400,
//
//                 title: 'temperature',
//
//                 drawOrder: ['series'],
//                 scales: {
//                     x: { time: true}
//                 },
//
//                 series: [
//                     {}, {
//                         stroke: '#000000',
//                         fill: '#00000000',
//                     }
//                 ]
//             }
//
//             tempChart = new uPlot(options, dataset, document.querySelector('.tempCanvas'))
//
//             // set the data in the legend
//             // set the dates
//             let minDateStr = minDate.getDate() + ' ' + minDate.getMonth() + ' ' + minDate.getFullYear()
//             let maxDateStr = maxDate.getDate() + ' ' + maxDate.getMonth() + ' ' + maxDate.getFullYear()
//             document.querySelector('.measurement.temp .maxDate').innerHTML = minDateStr
//             document.querySelector('.measurement.temp .minDate').innerHTML = maxDateStr
//             document.querySelector('.measurement.temp .xMax').innerHTML = Math.round(minVal)
//             document.querySelector('.measurement.temp .xMin').innerHTML = Math.round(maxVal)
//         })
//
//         fetch('/api/humidity')
//         .then(res => res.json())
//         .then((temps) => {
//
//
//             const {dataset, minVal, maxVal, minDate, maxDate} = genGraphData(temps)
//
//             let options = {
//                 width: 1920,
//                 height: 400,
//
//                 title: 'humidity',
//
//                 drawOrder: ['series'],
//                 scales: {
//                     x: { time: true}
//                 },
//
//                 series: [
//                     {}, {
//                         stroke: '#000000',
//                         fill: '#00000000',
//                     }
//                 ]
//             }
//
//             humidChart = new uPlot(options, dataset, document.querySelector('.humidCanvas'))
//
//             // set the data in the legend
//             // set the dates
//             let minDateStr = minDate.getDate() + ' ' + minDate.getMonth() + ' ' + minDate.getFullYear()
//             let maxDateStr = maxDate.getDate() + ' ' + maxDate.getMonth() + ' ' + maxDate.getFullYear()
//             document.querySelector('.measurement.humid .maxDate').innerHTML = minDateStr
//             document.querySelector('.measurement.humid .minDate').innerHTML = maxDateStr
//             document.querySelector('.measurement.humid .xMax').innerHTML = Math.round(minVal)
//             document.querySelector('.measurement.humid .xMin').innerHTML = Math.round(maxVal)
//         })
//
//         fetch('/api/pressures')
//         .then(res => res.json())
//         .then((temps) => {
//
//
//             const {dataset, minVal, maxVal, minDate, maxDate} = genGraphData(temps)
//
//             let options = {
//                 width: 1920,
//                 height: 400,
//
//                 title: 'pressures',
//
//                 drawOrder: ['series'],
//                 scales: {
//                     x: { time: true}
//                 },
//
//                 series: [
//                     {}, {
//                         stroke: '#000000',
//                         fill: '#00000000',
//                     }
//                 ]
//             }
//
//             pressChart = new uPlot(options, dataset, document.querySelector('.pressCanvas'))
//
//             // set the data in the legend
//             // set the dates
//             let minDateStr = minDate.getDate() + ' ' + minDate.getMonth() + ' ' + minDate.getFullYear()
//             let maxDateStr = maxDate.getDate() + ' ' + maxDate.getMonth() + ' ' + maxDate.getFullYear()
//             document.querySelector('.measurement.press .maxDate').innerHTML = minDateStr
//             document.querySelector('.measurement.press .minDate').innerHTML = maxDateStr
//             document.querySelector('.measurement.press .xMax').innerHTML = Math.round(minVal)
//             document.querySelector('.measurement.press .xMin').innerHTML = Math.round(maxVal)
//         })
//
// }
// let u;
//
//
//
//
//
// function getSize() {
//     return {
//         width: window.innerWidth - 100,
//         height: 600,
//     }
// }
