package main

import (
	"encoding/csv"
	"fmt"
	"math"
	"os"
	"time"
)

func main() {
	fmt.Println("Ejecutando pruebas de rendimiento...")

	nombreArchivoCSV := "testOperacionesGo.csv"
	archivoCSV, _ := os.Create(nombreArchivoCSV)
	defer archivoCSV.Close()

	writer := csv.NewWriter(archivoCSV)
	defer writer.Flush()

	writer.Write([]string{"Operación", "Repetición", "Tiempo (0.0001s)"})

	pruebaRendimiento(writer, "Suma", sumar)
	pruebaRendimiento(writer, "Resta", restar)
	pruebaRendimiento(writer, "RaizCuadrada", raizCuadrada)
	pruebaRendimiento(writer, "Multiplicacion", multiplicar)
	pruebaRendimiento(writer, "Division", dividir)
	pruebaRendimiento(writer, "Seno", seno)
	pruebaRendimiento(writer, "Coseno", coseno)

	fmt.Println("Pruebas terminadas")
}

func pruebaRendimiento(writer *csv.Writer, operacion string, funcionOperacion func()) {
	numRepeticiones := 5000
	numIteraciones := 100000000
	duraciones := make([]float64, numRepeticiones)

	for i := 0; i < numRepeticiones; i++ {
		tiempoInicio := time.Now()
		for i := 0; i < numIteraciones; i++ {
			funcionOperacion()
		}
		elapsed := time.Since(tiempoInicio).Seconds() * 10000 // Convertir a décimas de segundo con cuatro decimales
		duraciones[i] = elapsed

		fmt.Printf("%s - Repetición %d: %.4f (0.0001s)\n", operacion, i+1, elapsed)
		writer.Write([]string{operacion, fmt.Sprintf("%d", i+1), fmt.Sprintf("%.4f", elapsed)})
	}
}

func sumar() {
	_ = 2 + 3
}

func restar() {
	_ = 5 - 3
}

func raizCuadrada() {
	_ = math.Sqrt(16)
}

func multiplicar() {
	_ = 4 * 6
}

func dividir() {
	_ = 10.0 / 2.0
}

func seno() {
	_ = math.Sin(0.5)
}

func coseno() {
	_ = math.Cos(0.5)
}
