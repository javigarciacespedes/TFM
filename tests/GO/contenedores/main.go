package main

import (
	"encoding/csv"
	"fmt"
	"os"
	"os/exec"
	"strings"
	"time"
)

func crearYEliminarContenedor() (time.Duration, error) {
	// Crear el contenedor
	cmdCrear := exec.Command("docker", "run", "-d", "nginx:latest")
	tiempoInicio := time.Now() // Registrar el tiempo de inicio
	salidaCrear, err := cmdCrear.Output()
	if err != nil {
		return 0, fmt.Errorf("Error al crear el contenedor: %v", err)
	}
	IDContenedor := strings.TrimSpace(string(salidaCrear))

	// Calcular el tiempo de creación del contenedor
	transcurrido := time.Since(tiempoInicio)

	// Eliminar el contenedor
	cmdEliminar := exec.Command("docker", "container", "stop", IDContenedor)
	err = cmdEliminar.Run()
	if err != nil {
		return 0, fmt.Errorf("Error al detener el contenedor: %v", err)
	}
	return transcurrido, nil
}

func main() {
	numIteraciones := 100
	tiempoActual := time.Now()
	nombreArchivoCSV := fmt.Sprintf("testContGo_%s.csv", tiempoActual.Format("20060102150405"))

	archivoCSV, err := os.Create(nombreArchivoCSV)
	if err != nil {
		fmt.Printf("Error al crear el archivo CSV: %v\n", err)
		return
	}
	defer archivoCSV.Close()

	writer := csv.NewWriter(archivoCSV)
	defer writer.Flush()

	fmt.Println("Ejecutando pruebas de creación de contenedor...")

	// Escribir encabezado en el CSV
	writer.Write([]string{"Iteración", "Tiempo Transcurrido (ns)"})

	for i := 0; i < numIteraciones; i++ {
		transcurrido, err := crearYEliminarContenedor()
		if err != nil {
			fmt.Printf("Error en la iteración %d: %v\n", i+1, err)
		} else {
			fmt.Printf("Iteración %d: %s\n", i+1, transcurrido)
			writer.Write([]string{fmt.Sprintf("%d", i+1), fmt.Sprintf("%d", transcurrido.Nanoseconds())})
		}
	}

	fmt.Printf("Pruebas completadas. Resultados guardados en %s\n", nombreArchivoCSV)
}
