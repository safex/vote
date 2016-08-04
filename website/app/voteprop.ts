///<reference path="../node_modules/angular2/typings/browser.d.ts"/>

import {bootstrap} from 'angular2/platform/browser';
import {Component} from 'angular2/core';
import {Http, Response} from "angular2/http";
import 'rxjs/add/operator/map';
import {HTTP_PROVIDERS} from "angular2/http";

@Component({
	selector: 'voteproposal',
	template: `
		<h1>Safe Exchange | Submit Vote</h1>
		<a href="/">View Proposals</a>

		<br><input type="file" (change)="changeListener($event)" #input/>

		<br>{{ what_happen }}

		
	`,
	directives: [],
	styleUrls: []
})

export class AppComponent {
	what_happen: string;
	title: string;
	the_terms: string;
	responses: string[] = [];
	origin_pubkey: string;
	
    constructor(private _http: Http) {}

	upload_vote(body) {
		return this._http.post('http://localhost:3100/upload_vote', body)
			.map(res => res.json())
	}
 	

	changeListener(event) {
		var self = this;
		var contents = "";
		var reader = new FileReader();
		
        reader.onload = function(e:any) {
			try {contents = JSON.parse(e.target.result);
				console.log(JSON.stringify(contents));
       		self.upload_vote(JSON.stringify(contents))
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

    open_proposal(body) {
		return this._http.post('http://localhost:3100/return_proposal', body)
			.map(res => res.json())
	}

	loadProposal(stringified) {
		var self = this;
		this.open_proposal(stringified)
			.subscribe(
				data => { self.title = data.title;
					self.the_terms = data.the_terms;
					self.responses = data.responses;
					self.origin_pubkey = data.origin_pubkey;	
				},
				error => console.log("error"),
				() => console.log("finished")
			);
	}

    ngOnInit() {
		var currentLocation = window.location;
		console.log(currentLocation);
		var res = currentLocation.pathname.split("/voteproposal/");
		console.log(res[1]);
		var result = res[1];
		var key = "directory_name";
		var json = {};
		json[key] = result;
		var stringified = JSON.stringify(json);
		this.loadProposal(stringified);
	}
}

bootstrap(AppComponent, [HTTP_PROVIDERS]);
