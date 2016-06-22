import {bootstrap} from "@angular/platform-browser-dynamic";
import {Component} from "@angular/core";
import {ImportComponent} from "./import.component";
import {HTTP_PROVIDERS} from "@angular/http";

@Component({
	selector: 'app',
	template: `<h1>hello world</h1>
	<import></import>
	`,
	directives: [ImportComponent]
})

export class AppComponent {}

bootstrap(AppComponent, [HTTP_PROVIDERS]);