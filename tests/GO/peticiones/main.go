package main

import (
	"fmt"
	"net/http"
)

func main() {
	http.HandleFunc("/", handler)
	port := 8080
	fmt.Printf("Servidor escuchando en el puerto %d...\n", port)
	err := http.ListenAndServe(fmt.Sprintf(":%d", port), nil)
	if err != nil {
		fmt.Println("Error:", err)
	}
}

func handler(w http.ResponseWriter, r *http.Request) {
	switch r.Method {
	case http.MethodGet:
		handleGet(w, r)
	case http.MethodPost:
		handlePost(w, r)
	default:
		http.Error(w, "Método no soportado", http.StatusMethodNotAllowed)
	}
}

func handleGet(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintln(w, "Respuesta de GET")
}

func handlePost(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintln(w, "Respuesta de POST")
}
