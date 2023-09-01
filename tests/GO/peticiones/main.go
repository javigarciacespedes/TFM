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
	urlGET := "https://jsonplaceholder.typicode.com/posts/1"
	urlPOST := "https://jsonplaceholder.typicode.com/posts"

	archivo, err := os.Create("resultados.csv")
	if err != nil {
		fmt.Println("Error al crear el archivo CSV:", err)
		return
	}
	defer archivo.Close()

	escritorCSV := csv.NewWriter(archivo)
	defer escritorCSV.Flush()

	escritorCSV.Write([]string{"Iteración", "Tipo", "Tiempo (s)"})

	for i := 0; i < 10; i++ {
		// GET
		duracionTotalGET := time.Duration(0)
		for j := 0; j < 10; j++ { // Ejecutar cada operación 10 veces
			inicioGET := time.Now()
			_, err := http.Get(urlGET)
			if err != nil {
				fmt.Println("Error en la petición GET:", err)
				return
			}
			finGET := time.Now()
			duracionGET := finGET.Sub(inicioGET)
			duracionTotalGET += duracionGET
		}
		duracionSegundosGET := float64(duracionTotalGET) / float64(time.Second)
		duracionConPrecisionGET := fmt.Sprintf("%.4f", duracionSegundosGET)
		escritorCSV.Write([]string{fmt.Sprintf("%d", i+1), "GET", duracionConPrecisionGET})
		fmt.Printf("Iteración %d: Operación GET registrada - Tiempo = %s\n", i+1, duracionConPrecisionGET)

		// POST
		duracionTotalPOST := time.Duration(0)
		for j := 0; j < 10; j++ { // Ejecutar cada operación 10 veces
			inicioPOST := time.Now()
			_, err := http.Post(urlPOST, "application/json", bytes.NewBuffer([]byte(`{"title": "foo", "body": "bar", "userId": 1}`)))
			if err != nil {
				fmt.Println("Error en la petición POST:", err)
				return
			}
			finPOST := time.Now()
			duracionPOST := finPOST.Sub(inicioPOST)
			duracionTotalPOST += duracionPOST
		}
		duracionSegundosPOST := float64(duracionTotalPOST) / float64(time.Second)
		duracionConPrecisionPOST := fmt.Sprintf("%.4f", duracionSegundosPOST)
		escritorCSV.Write([]string{fmt.Sprintf("%d", i+1), "POST", duracionConPrecisionPOST})
		fmt.Printf("Iteración %d: Operación POST registrada - Tiempo = %s\n", i+1, duracionConPrecisionPOST)
	}

	fmt.Println("Pruebas completadas. Resultados almacenados en resultados.csv")
}
