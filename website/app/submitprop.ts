///<reference path="../node_modules/angular2/typings/browser.d.ts"/>

import {bootstrap} from 'angular2/platform/browser';
import {Component} from 'angular2/core';
import {Http, Response} from "angular2/http";
import 'rxjs/add/operator/map';
import {HTTP_PROVIDERS} from "angular2/http";

@Component({
	selector: 'submitprop',
	template: `

		<div class="box">
		<div class="header-container">
		<div class="logo">
			<img src="img/safex-logo.png">
		</div>
			<h1 class="main-head">Safe Exchange | Submit Proposal</h1>
		</div>

	    <div class="outer-box">

	    	<a href="/"><button class="small-btn first">View Proposals</button></a>

	    	<br><input type="file" (change)="changeListener($event)" #input/>
		<br>{{ what_happen }}
		
   	
	</div>
	</div>

		
	`,
	directives: [],
	styleUrls: ['css/submitprop/css/bootstrap.css', 'css/submitprop/css/style.css']
})

export class AppComponent {
	what_happen: string;
	
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
			try {contents = JSON.parse(e.target.result);
				console.log(JSON.stringify(contents));
       		self.upload_proposal(JSON.stringify(contents))
       			.subscribe(
       				data => self.what_happen = JSON.stringify(data),
       				error => self.what_happen = "error with your file",
       				() => console.log("finished subscribe")
       			);
       			} catch (Error) {
    		self.what_happen = "error";
    	}
        };

        // read the image file as a data URL.
        reader.readAsText(event.target.files[0]);
    }

}

bootstrap(AppComponent, [HTTP_PROVIDERS]);
