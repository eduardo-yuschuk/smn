# -*- coding: utf-8 -*-

#days = 1
days = int(input("Buscamos el pronóstico a 1, 2 o 3 días?:"))
print("Buscando el pronóstico a %d día/s..." % days)
url = format("https://ws.smn.gob.ar/map_items/forecast/%d" % days)

#import json
import requests
response = requests.get(url)
if response.status_code != 200:
    print("Error leyendo del api REST")
    exit(1)

"""
{
    '_id': '5ec529d3364a909f8350af8f', 
    'dist': 13.66, 
    'lid': 4875, 
    'fid': 4875, 
    'name': 'Lomas de Zamora', 
    'province': 'Buenos Aires', 
    'lat': '-34.7586441', 
    'lon': '-58.40148926', 
    'zoom': '2', 
    'updated': 1557347400, 
    'weather': {
        'day': 1, 
        'morning_temp': 15, 
        'morning_id': 18, 
        'morning_desc': 'Cielo parcialmente nublado a nublado. Probabilidad neblinas especialmente en la zona suburbana. Tiempo desmejorando,  con probabilidad de lluvias  y algunas tormentas. Vientos moderados del sector este.', 
        'afternoon_temp': 18, 
        'afternoon_id': 4, 
        'afternoon_desc': 'Cielo nublado. Probabilidad de lluvias y tormentas. Vientos   leves del sector este.'
    }
}
"""
    
print("Seleccione una provincia (ingrese el número):")
provinces = [report['province'] for report in response.json()]
provinces = list(set(provinces))
provinces.sort()
for id, province in enumerate(provinces):
    print("[%d] %s" % (id, province))
province_id = int(input(">"))
province = provinces[province_id]
print("Provincia seleccionada: %s..." % province)

print("Seleccione una ciudad (ingrese el número):")
cities = [report['name'] for report in response.json() if report['province'] == province]
cities = list(set(cities))
cities.sort()
for id, city in enumerate(cities):
    print("[%d] %s" % (id, city))
city_id = int(input(">"))
city = cities[city_id]
print("Ciudad seleccionada: %s..." % city)

for weather in [report['weather'] for report in response.json() if report['province'] == province and report['name'] == city]:
    print("Ma~ana:")
    print("Temperatura: %d" % int(weather['morning_temp']))
    print("Pronostico: %s" % " ".join(weather['morning_desc'].split()))
    print("Tarde:")
    print("Temperatura: %d" % int(weather['afternoon_temp']))
    print("Pronostico: %s" % " ".join(weather['afternoon_desc'].split()))
    
