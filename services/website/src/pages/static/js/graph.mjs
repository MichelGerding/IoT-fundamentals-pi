import uPlot from '../uplot/main.mjs';

class Graph extends HTMLElement {
    updateState(newState) {
        let actualChange = false

        if (newState.type !== undefined && newState.type !== this.getAttribute('type')) {
            this.setAttribute('type', newState.type);
            actualChange = true;
        }

        console.log(newState.dateRange, this.getAttribute('dateRange'))
        if (newState.dateRange !== undefined && newState.dateRange !== this.getAttribute('dateRange')) {
            this.setAttribute('dateRange', newState.dateRange);
            actualChange = true;
        }

        console.log(actualChange, "state changed")
        actualChange && this.#stateChanged();
    }



    constructor() {
        super();
        this.shadow = this.attachShadow({mode: 'open'});

        let daterange = document.getElementById('datepicker').value;
        this.setAttribute('dateRange', daterange);

        this.#stateChanged();
    }


    #stateChanged() {
        let dates = this.getAttribute('dateRange').split(' - ');

        // convert the dates to unix timestamps
        let dateRangeB64 = btoa(`${new Date(dates[0]).getTime() / 1000}-${new Date(dates[1]).getTime() / 1000}`)
        console.log(dateRangeB64)




        // get the dat from the api
        fetch(`/api/${this.getAttribute('type').toLowerCase()}/${dateRangeB64}`)
            .then(response => response.json())
            .then(data => {
                // transform the data into the expected format
                // [[x1, x2, x3, ...], [y1, y2, y3, ...]]

                let dx = []
                let dy = data.map(d => {
                    dx.push(d.timestamp)
                    return d.value
                })

                this.data = [dx, dy]

                this.#render();
                this.#renderPlot();
            })
            .catch(err => {
                console.log(err);
                this.data = [[], []];
                this.#render();
            })
    }

    #renderPlot() {

        console.log(this.shadow.querySelector('#graph').clientWidth)

        let options = {
            width: this.shadow.querySelector('#graph').clientWidth,
            height: 315 + 65,

            axes: [
                {
                    tick: {
                        show: false,
                    },
                }
            ],

            drawOrder: ['series'],

            scales: {
                x: { time: true}
            },

            series: [
                {}, {
                    stroke: '#000000',
                    fill: '#00000000',
                    thickness: 2
                }]
        }

        let plot = new uPlot(options, this.data, this.shadow.querySelector('#graph'))

        console.log(plot)

    }


    #render() {

        const type = this.getAttribute('type').toUpperCase();

        // generate the background text
        // which contains 3 rows with each word 3 times with different offsets
        // and jsut store each row in an array

        // get only the first half of the type
        const half = Math.floor(type.length / 2);
        const firstHalf = type.slice(0, half);
        const secondHalf = type.slice(half, type.length);

        const bg = `
            <div class="bg" style="color: rgba(0,0,0,0.1); font-size: 175px; letter-spacing: -0.1em; line-height: 0.6em">
                <span class="word">${type}${type}${type}</span>
                <span class="word">${secondHalf}<span STYLE="font-weight: bold; color: rgba(0,0,0,0.2)">${type}</span>${firstHalf}</span>
                <span class="word">${firstHalf}${secondHalf}${type}</span>
            </div>
        `


        if (type === undefined) {
            this.shadow.innerHTML = `
                <h2>Invalid type</h2>
            `;
            return;
        }

        let minDate = new Date(this.data[0][0] * 1000).toLocaleTimeString("nl-NL",{
            day: 'numeric',
            month: 'short',
            year: 'numeric',
            hour: 'numeric',
            minute: 'numeric',
            hour12: false
            });
        let maxDate = new Date(this.data[0][this.data[0].length - 1] * 1000).toLocaleString("nl-NL",{
            day: 'numeric',
            month: 'short',
            year: 'numeric',
            hour: 'numeric',
            minute: 'numeric',
            hour12: false
        });;

        if (this.data.length === 0) {

        }


        this.shadow.innerHTML = `
            <style>
                @import url('/static/styles/uplot.min.css');
                .container {
                    position: relative;
                    width: calc(100vw - 80px);
                    height: 315px;
                    
                    margin-bottom: 200px;
                }
                .bg {
                    position: absolute;
                    opacity: 0.2;
                    top: 0;
                    left: 50%;
                    transform: translateX(-50%);                    
                }
                
                .graph {
                    position: relative;
                    height: 100%;
                }
                
                #graph {
                    width: calc(100% - 40px) ;
                    height: 315px;
                }
                
                .left, .right {
                    position: absolute;
                
                    width: 315px;
                    height: 20px;
                    
                    display: flex;
                    flex-direction: row;
    
                    justify-content: space-between;

                }

                .left {
                    bottom: -20px;
                    left: 0;
                    
                    transform: rotate(-90deg);    
                    transform-origin: top left;             
                }
                
                .right {
                    top: -20px;
                    right: 0;
                    
                    transform: rotate(-90deg);
                    transform-origin: bottom right;
                }
                
                
            </style>
                <h4> ${type} </h4>    
            <div class="container">
                ${bg}
                
                <div class="graph"> 
                ${this.data[0].length > 0 ? `<div class="left">
                    <span class="minx">${minDate}</span>
                        <span class="maxy">${Math.max.apply(null, this.data[1])}</span>
                    </div>`:''}
                    
            
                        <div id="graph"></div>
                    
                ${this.data[0].length > 0 ? `
                    <div class="right">
                        <span class="miny">${Math.min.apply(null, this.data[1])}</span>
                        <span class="maxx">${maxDate}</span>
                    </div>`: ''}
                </div>
            </div>
        `
    }
}

customElements.define('graph-component', Graph);
