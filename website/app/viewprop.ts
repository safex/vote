///<reference path="../node_modules/angular2/typings/browser.d.ts"/>

import {bootstrap} from 'angular2/platform/browser';
import {Component} from 'angular2/core';
import {Http} from "angular2/http";
import 'rxjs/add/operator/map';
import {HTTP_PROVIDERS} from "angular2/http";



@Component({
	selector: 'proposalview',
	template: `
		<h1>Safe Exchange | Proposal View</h1>
		<a href="/">Home</a>
		<a href="/submitprop">Submit Proposal</a>

		<br>Title: {{ title }}
		<br>The Terms: {{ the_terms }}
		<br>Responses: 
			<ul>
				<li *ngFor="let resp of responses">
					{{ resp }}
				</li>
			</ul>
		<br>Origin Public Key: {{ origin_pubkey }}
	`,
	directives: [],
	styleUrls: []
})

export class ViewProposalComponent {
	title: string;
	the_terms: string;
	responses: string[] = [];
	origin_pubkey: string;

    constructor(private _http: Http) {}

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
		var res = currentLocation.pathname.split("/viewproposal/");
		console.log(res[1]);
		var result = res[1];
		var key = "directory_name";
		var json = {};
		json[key] = result;
		var stringified = JSON.stringify(json);
		this.loadProposal(stringified);
	}




}

bootstrap(ViewProposalComponent, [HTTP_PROVIDERS]);
