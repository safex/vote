///<reference path="../node_modules/angular2/typings/browser.d.ts"/>

import {bootstrap} from 'angular2/platform/browser';
import {Component} from 'angular2/core';

@Component({
	selector: 'app',
	template: `
		<h1>Safe Exchange Community</h1>
	`,
	directives: []
})

export class AppComponent {

}

bootstrap(AppComponent);