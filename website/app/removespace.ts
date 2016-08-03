//file name: remove-spaces.ts
import {Pipe} from "angular2/core";

@Pipe({
	name : "removeSpaces"
})

export class RemoveSpaces{
	transform(value){
		return value.replace(/ /g, "");
	}
}