///<reference path="../node_modules/angular2/typings/browser.d.ts"/>

import {bootstrap} from 'angular2/platform/browser';
import {Component} from 'angular2/core';
import {Http} from "angular2/http";
import 'rxjs/add/operator/map';
import {HTTP_PROVIDERS} from "angular2/http";

@Component({
	selector: 'submitprop',
	template: `
		<h1>Safe Exchange | Submit Proposal</h1>
		<a href="/">View Proposals</a>

		<input type="file" (change)="changeListener($event)" #input/>

		
	`,
	directives: [],
	styleUrls: []
})

export class AppComponent {
	
    constructor(private _http: Http) {}

	upload_proposal(body) {
		return this._http.post('http://localhost:3100/upload_proposal', body)
			.map(res => res.json())
	}


	changeListener(event) {
		var self = this;
		var contents = "";
		var reader = new FileReader();
        reader.onload = function(e:any) {
			contents = JSON.parse(e.target.result);
       		self.upload_proposal(JSON.stringify(contents))
       			.subscribe(
       				data => console.log("sent"),
       				error => console.log("error getting data here its fine"),
       				() => console.log("finished subscribe")
       			);
        };
        // read the image file as a data URL.
        reader.readAsText(event.target.files[0]);
    }

}

bootstrap(AppComponent, [HTTP_PROVIDERS]);
