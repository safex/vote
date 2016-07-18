///<reference path="../node_modules/angular2/typings/browser.d.ts"/>

import {bootstrap} from 'angular2/platform/browser';
import {Component} from 'angular2/core';

@Component({
	selector: 'app',
	template: `
		<h1>Safe Exchange Community Voting</h1>
		<a href="/">Home</a>
		<a href="/submitproposal">Submit Proposal</a>

	`,
	directives: []
})

export class AppComponent {

}

bootstrap(AppComponent);

//this app reads proposals and lists them to the front of the page