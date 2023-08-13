package main

import (
	"fmt"
	"log"
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
	archivoLog, err := os.Create("testContGo.log")
	if err != nil {
		log.Fatalf("Error al crear el archivo de registro: %v", err)
	}
	defer archivoLog.Close()

	fmt.Println("Ejecutando pruebas de creación de contenedor...")

	for i := 0; i < numIteraciones; i++ {
		transcurrido, err := crearYEliminarContenedor()
		if err != nil {
			log.Printf("Error en la iteración %d: %v", i+1, err)
		} else {
			fmt.Printf("Iteración %d: %s\n", i+1, transcurrido)
			fmt.Fprintf(archivoLog, "Iteración %d: %s\n", i+1, transcurrido)
		}
	}
}
