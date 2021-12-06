const http = require('http');

http.request({
    hostname: 'ws.smn.gob.ar',
    port: 80,
    path: '/map_items/forecast/1',
    method: 'GET',
}, (res) => {
    res.setEncoding('utf8');
    var content = "";
    res.on('data', (chunk) => {
        content += chunk;
    });
    res.on('end', () => {
        JSON.parse(content).forEach(report => {
            if (report['name'] == 'Morón') {
                console.log('');
                console.log('Mañana');
                console.log('------');
                console.log('Temperatura: ' + report['weather']['morning_temp']);
                console.log('Pronóstico: ' + report['weather']['morning_desc']);
                console.log('');
                console.log('Tarde');
                console.log('-----');
                console.log('Temperatura: ' + report['weather']['afternoon_temp']);
                console.log('Pronóstico: ' + report['weather']['afternoon_desc']);
                console.log('');
            }
        });
    });
}).end();