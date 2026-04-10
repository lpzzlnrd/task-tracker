//package main

import "fmt"
/*
func main(){

	var nota, suma, promedio float64
	const cantidadNotas= 5
	const notaAprobado= 20.0

	fmt.Println("Ingrese las notas a continuacion: ")

	for i:= 1; i<=cantidadNotas; i++{

		fmt.Printf("Nota %d:\n", i)
		fmt.Scanln(&nota)
		suma+= nota
	}

	promedio = suma / cantidadNotas

	fmt.Printf("El promedio final es: %.2f\n", promedio)

	// Indica si el estudiante aprobó o no
	if promedio >= notaAprobado {
		fmt.Println("El estudiante aprobó.")
	} else {
		fmt.Println("El estudiante no aprobó.")
	}

}
*/
package main

type Estudiante struct{

	Nombre string
	Notas []float64
}

func(e Estudiante) MostrarNotas(){
	fmt.Println("Notas de:", e.Nombre, ": ", e.Notas)
}

func(e Estudiante) CalcularPromedio()float64{
	suma:=0.0
	for _, nota := range e.Notas{
		suma+=nota
	}
	promedio := suma / float64(len(e.Notas))
	return promedio
}


func main(){
	notas:= []float64{18.5, 20.0, 15.0, 19.0, 17.5}
	resultado := CalcularPromedio(notas)
	Aprobado(resultado)
}
