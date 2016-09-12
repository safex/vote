///<reference path="../node_modules/angular2/typings/browser.d.ts"/>

import {bootstrap} from 'angular2/platform/browser';
import {Component} from 'angular2/core';
import {Http, Response} from "angular2/http";
import 'rxjs/add/operator/map';
import {HTTP_PROVIDERS} from "angular2/http";

@Component({
	selector: 'voteproposal',
	template: `
			<div class="box">
		<div class="header-container">
		<div class="logo">
			<img src="img/safex-logo.png">
		</div>
	<h1 class="main-head">Safe Exchange | Submit Vote</h1>
		</div>

	    <div class="outer-box">

	    	<a href="/"><button class="small-btn first">View Proposals</button></a>

		<br><input type="file" (change)="changeListener($event)" #input/>
		<br>{{ what_happen }}
		<br>
		<br>
		<br>
		Submitting Vote for:
		<br>
		<br>
		<br><p class="attribute">Title:</p> 
			<br>{{ title }}
		<br><p class="attribute">The Terms:</p>
			<br>{{ the_terms }}
		<br>
			<br><p class="attribute">Responses:</p>
			<ul>
				<li *ngFor="let resp of responses">
					{{ resp }}
				</li>
			</ul>

		
		
   	
	</div>
	</div>	

		
	`,
	directives: [],
	styleUrls: ['css/voteprop/css/bootstrap.css', 'css/voteprop/css/style.css']
})

export class AppComponent {
	what_happen: string;
	title: string;
	the_terms: string;
	responses: string[] = [];
	origin_pubkey: string;

	directoryname: string;
	
    constructor(private _http: Http) {}

	upload_vote(body) {
		return this._http.post('http://localhost:3100/upload_vote', body)
			.map(res => res.json())
	}
 	

 	//we have to send a vote and tell the service which proposal we are voting on.

	changeListener(event) {
		var self = this;
		var contents = "";
		var reader = new FileReader();
		
        reader.onload = function(e:any) {
			try {contents = JSON.parse(e.target.result);
				var key1 = "vote";
				var key2 = "proposal_directory";
				var json = {};
				json[key1] = contents;
				json[key2] = self.directoryname;
				console.log(JSON.stringify(json));
       		self.upload_vote(JSON.stringify(json))
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
		this.directoryname = res[1];
	}
}

bootstrap(AppComponent, [HTTP_PROVIDERS]);
