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

	nombreArchivoCSV := "testMathGo.csv"
	archivoCSV, _ := os.Create(nombreArchivoCSV)
	defer archivoCSV.Close()

	writer := csv.NewWriter(archivoCSV)
	defer writer.Flush()

	writer.Write([]string{"Operación", "Repetición", "Tiempo (ns)"})

	// Iteraciones de calentamiento
	calentar(sumar)
	calentar(restar)
	calentar(raizCuadrada)
	calentar(multiplicar)
	calentar(dividir)
	calentar(seno)
	calentar(coseno)

	// Mediciones reales
	pruebaRendimiento(writer, "Suma", sumar)
	pruebaRendimiento(writer, "Resta", restar)
	pruebaRendimiento(writer, "RaizCuadrada", raizCuadrada)
	pruebaRendimiento(writer, "Multiplicacion", multiplicar)
	pruebaRendimiento(writer, "Division", dividir)
	pruebaRendimiento(writer, "Seno", seno)
	pruebaRendimiento(writer, "Coseno", coseno)
}

func calentar(funcionOperacion func()) {
	numRepeticiones := 20
	for i := 0; i < numRepeticiones; i++ {
		funcionOperacion()
	}
}

func pruebaRendimiento(writer *csv.Writer, operacion string, funcionOperacion func()) {
	numRepeticiones := 100
	duraciones := make([]time.Duration, numRepeticiones)

	for i := 0; i < numRepeticiones; i++ {
		tiempoInicio := time.Now()
		funcionOperacion()
		elapsed := time.Since(tiempoInicio)
		duraciones[i] = elapsed

		fmt.Printf("%s - Repetición %d: %s\n", operacion, i+1, elapsed)
		writer.Write([]string{operacion, fmt.Sprintf("%d", i+1), fmt.Sprintf("%d", elapsed.Nanoseconds())})
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
