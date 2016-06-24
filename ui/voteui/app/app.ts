///<reference path="../node_modules/angular2/typings/browser.d.ts"/>

import {bootstrap} from 'angular2/platform/browser';
import {Component} from 'angular2/core';
import {VoteComponent} from './vote.component.ts';
import {HTTP_PROVIDERS} from "angular2/http";


@Component({
	selector: 'app',
	template: `
		<h2>hello world</h2>
		<vote></vote>
	`,
	directives: [VoteComponent]
})

export class AppComponent {

}

bootstrap(AppComponent, [HTTP_PROVIDERS]);