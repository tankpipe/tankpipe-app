export const chartOptions = {
    series: [
        {
            name: "foo",
            data: [],
        },
    ],
    chart: {
        type: "area",
        height: 50,
        width: 100,
        sparkline: {
            enabled: true,
        },
    },
    stroke: {
        curve: "stepline",
        width: 2,
        colors:["#efefef"],
    },
    fill: {
        opacity: 0.6,
        colors: ["#efefef"],
    },
    xaxis: {
        crosshairs: {
            width: 1,
        },
        type: "datetime",
    },
    tooltip: {
        enabled: false
    },
}