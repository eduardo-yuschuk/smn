package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"os"
)

/*
[
    {
        "_id": "5ec3d853364a909f83506c71",
        "dist": 3.93,
        "lid": 6002,
        "fid": 6002,
        "name": "Villa de María",
        "province": "Córdoba",
        "lat": "-29.90564728",
        "lon": "-63.72361755",
        "zoom": "2",
        "updated": 1557349200,
        "weather": {
            "day": 1,
            "morning_temp": 14,
            "morning_id": 11,
            "morning_desc": "Nubosidad variable. Vientos leves del sector norte. ",
            "afternoon_temp": 22,
            "afternoon_id": 14,
            "afternoon_desc": "Nubosidad variable.  Probabilidad de lluvias y tormentas. Vientos leves del sector norte. "
        }
	},
...
*/

// Response ...
type Response struct {
	Reports []Report
}

// Report ...
type Report struct {
	ID       string  `json:"_id"`
	Dist     float64 `json:"dist"`
	Lid      int     `json:"lid"`
	Fid      int     `json:"fid"`
	Name     string  `json:"name"`
	Province string  `json:"province"`
	Lat      float64 `json:"lat"`
	Lon      float64 `json:"lon"`
	Zoom     int     `json:"zoom"`
	Updated  int     `json:"updated"`
	Weather  Weather `json:"weather"`
}

// Weather ...
type Weather struct {
	Day           int    `json:"day"`
	MorningTemp   int    `json:"morning_temp"`
	MorningID     int    `json:"morning_id"`
	MorningDesc   string `json:"morning_desc"`
	AfternoonTemp int    `json:"afternoon_temp"`
	AfternoonID   int    `json:"afternoon_id"`
	AfternoonDesc string `json:"afternoon_desc"`
}

func main() {
	days := 1
	url := fmt.Sprintf("https://ws.smn.gob.ar/map_items/forecast/%d", days)
	fmt.Println(url)
	response, err := http.Get(url)

	if err != nil {
		fmt.Print(err.Error())
		os.Exit(1)
	}

	responseData, err := ioutil.ReadAll(response.Body)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println(string(responseData[0:1000]))

	var responseObject Response
	err = json.Unmarshal(responseData, &responseObject)
	if err != nil {
		fmt.Print(err.Error())
		os.Exit(1)
	}

	fmt.Println(responseObject)

	/*for i := 0; i < len(responseObject.Reports); i++ {
		fmt.Println(responseObject.Reports[i].ID)
	}*/
}
