// clase de 7-3-2026

package main
import "fmt"


type Persona struct{
	
	Nombre string
	Edad int
}


func (p Persona) Saludar(){
	fmt.Println("Hola soy", p.Nombre)
}


func (p Persona) CalcularEdad(edad int){

	//var edad int
	fmt.Println("HOLA METE TU EDAD AQui: ")
	
	fmt.Scanf("%d", &edad)


	if edad >=18{
		fmt.Println("eres mayor de edad XD")
	} else if edad < 0 {
		fmt.Println("edad invalida")
	}

	return

}

func main(){

	p1 := Persona{Nombre: "Juan", Edad: 30}

	p1.Saludar()
	p1.CalcularEdad(p1.Edad)
}



/*

en GO, no ecisten las clases por concepto al estilo de Java,
ni herencia clasica. la programacion orientada a objetos usa la declaracion
"structs" que estructura datos agrupando variables condicionadas, ES
el reemplazo de clases en GO.

*/