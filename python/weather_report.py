import requests
weather = [report['weather'] for report in requests.get("http://ws.smn.gob.ar/map_items/forecast/1").json() if report['name'] == 'Morón'][0]
print("Mañana:\nTemperatura: %d" % int(weather['morning_temp']))
print("Pronóstico: %s" % " ".join(weather['morning_desc'].split()))
print("Tarde:\nTemperatura: %d" % int(weather['afternoon_temp']))
print("Pronóstico: %s" % " ".join(weather['afternoon_desc'].split()))
