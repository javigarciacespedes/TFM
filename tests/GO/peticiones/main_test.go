package main

import (
	"bytes"
	"encoding/csv"
	"fmt"
	"net/http"
	"os"
	"time"
)

func main() {
	getURL := "https://jsonplaceholder.typicode.com/posts/1"
	postURL := "https://jsonplaceholder.typicode.com/posts"

	file, err := os.Create("resultados.csv")
	if err != nil {
		fmt.Println("Error al crear el archivo CSV:", err)
		return
	}
	defer file.Close()

	writer := csv.NewWriter(file)
	defer writer.Flush()

	writer.Write([]string{"Tipo", "Tiempo"})

	for i := 0; i < 10; i++ {
		// Medir tiempo para GET
		getStartTime := time.Now()
		_, err := http.Get(getURL)
		getEndTime := time.Now()

		if err != nil {
			fmt.Println("Error en la petición GET:", err)
			return
		}

		getDuration := getEndTime.Sub(getStartTime)
		writer.Write([]string{"GET", getDuration.String()})

		// Medir tiempo para POST
		postStartTime := time.Now()
		_, err = http.Post(postURL, "application/json", bytes.NewBuffer([]byte(`{"title": "foo", "body": "bar", "userId": 1}`)))
		postEndTime := time.Now()

		if err != nil {
			fmt.Println("Error en la petición POST:", err)
			return
		}

		postDuration := postEndTime.Sub(postStartTime)
		writer.Write([]string{"POST", postDuration.String()})
	}

	fmt.Println("Pruebas completadas. Resultados almacenados en resultados.csv")
}
