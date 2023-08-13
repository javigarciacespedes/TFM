package main

import (
	"fmt"
	"math"
	"os"
	"time"
)

func main() {
	fmt.Println("Ejecutando pruebas de rendimiento...")
	os.Remove("testMathGo.log") // Elimina el archivo si ya existe

	archivo, _ := os.Create("testMathGo.log")
	defer archivo.Close()

	pruebaRendimiento(archivo, "Suma", sumar)
	pruebaRendimiento(archivo, "Resta", restar)
	pruebaRendimiento(archivo, "RaizCuadrada", raizCuadrada)
	pruebaRendimiento(archivo, "Multiplicacion", multiplicar)
	pruebaRendimiento(archivo, "Division", dividir)
	pruebaRendimiento(archivo, "Seno", seno)
	pruebaRendimiento(archivo, "Coseno", coseno)
}

func pruebaRendimiento(archivo *os.File, operacion string, funcionOperacion func()) {
	numRepeticiones := 100
	duraciones := make([]time.Duration, numRepeticiones)

	for i := 0; i < numRepeticiones; i++ {
		tiempoInicio := time.Now()
		funcionOperacion()
		elapsed := time.Since(tiempoInicio)
		duraciones[i] = elapsed

		fmt.Fprintf(archivo, "%s - Repetición %d: %s\n", operacion, i+1, elapsed)
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
