import pandas as pd
import matplotlib.pyplot as plt
import os

def cargarDatosOperaciones(archivo_csv):
    data = pd.read_csv(archivo_csv, sep=';', header=0, names=['Operacion', 'Iteracion', 'Tiempo'])
    data['Tiempo'] = data['Tiempo'].str.replace(',', '.').astype(float)
    return data

def cargarDatosPeticiones(archivo_csv):
    data = pd.read_csv(archivo_csv, sep=';', header=0, names=['Iteracion', 'Operacion', 'Tiempo'])
    data['Tiempo'] = data['Tiempo'].str.replace(',', '.').astype(float)
    return data

def analizarOperacion(data, operacion, archivo_csv):
    resultados = data[data['Operacion'] == operacion].copy()
    
    media_tiempo = resultados['Tiempo'].mean()
    valor_alto = resultados['Tiempo'].max()
    valor_bajo = resultados['Tiempo'].min()
    
    print(f"{operacion.upper()}: {archivo_csv}")
    print(f"El tiempo medio de ejecución es de: {media_tiempo} milésimas")
    print(f"El tiempo de ejecución más alto ha sido de {valor_alto} milésimas. Y el de menor valor es de {valor_bajo} milésimas.")
    
    plt.figure(figsize=(10, 6))
    _, _, _ = plt.hist(resultados['Tiempo'], bins=20, edgecolor='k', range=(resultados['Tiempo'].min(), resultados['Tiempo'].max()))
    plt.xlabel('Tiempo (milesimas)')
    plt.ylabel('Frecuencia')
    plt.title(f'Histograma de Tiempos de Ejecución - {operacion}')
    plt.grid(True)
    plt.text(0.05, 0.95, f'Archivo: {archivo_csv}', transform=plt.gca().transAxes, fontsize=8, verticalalignment='top', bbox=dict(boxstyle='round, pad=0.3', facecolor='lightgray', alpha=0.7))
    plt.show()

def analizarPeticion(data, operacion, archivo_csv):
    resultados = data[data['Operacion'] == operacion].copy()
    
    resultados = resultados[resultados['Tiempo'] <= 60]

    media_tiempo = resultados['Tiempo'].mean()
    valor_alto = resultados['Tiempo'].max()
    valor_bajo = resultados['Tiempo'].min()
    
    print(f"{operacion.upper()}: {archivo_csv}")
    print(f"El tiempo medio de ejecución es de: {media_tiempo} segundos")
    print(f"El tiempo de ejecución más alto ha sido de {valor_alto} segundos. Y el de menor valor es de {valor_bajo} segundos.")
    
    plt.figure(figsize=(10, 6))
    _, _, _ = plt.hist(resultados['Tiempo'], bins=20, edgecolor='k', range=(resultados['Tiempo'].min(), resultados['Tiempo'].max()))
    plt.xlabel('Tiempo (segundos)')
    plt.ylabel('Frecuencia')
    plt.title(f'Histograma de Tiempos de Ejecución - {operacion}')
    plt.grid(True)
    plt.text(0.05, 0.95, f'Archivo: {archivo_csv}', transform=plt.gca().transAxes, fontsize=8, verticalalignment='top', bbox=dict(boxstyle='round, pad=0.3', facecolor='lightgray', alpha=0.7))
    plt.show()

def analizarArchivos():
    directorio_actual = os.getcwd()
    archivos_en_directorio = os.listdir(directorio_actual)
    
    archivos_csv_operaciones = [archivo for archivo in archivos_en_directorio if 'Operaciones' in archivo and archivo.endswith('.csv')]
    archivos_csv_peticiones = [archivo for archivo in archivos_en_directorio if 'Peticiones' in archivo and archivo.endswith('.csv')]

    for archivo_csv in archivos_csv_operaciones:
        data = cargarDatosOperaciones(archivo_csv)
        operaciones = ['Suma', 'Resta', 'Multiplicacion', 'Division', 'RaizCuadrada', 'Seno', 'Coseno']
        
        for operacion in operaciones:
            analizarOperacion(data, operacion, archivo_csv)

    for archivo_csv in archivos_csv_peticiones:
        data = cargarDatosPeticiones(archivo_csv)
        operaciones = ['GET', 'POST']
        
        for operacion in operaciones:
            analizarPeticion(data, operacion, archivo_csv)

# Llamar a la función para analizar los archivos CSV en el directorio actual
analizarArchivos()
