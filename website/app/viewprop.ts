///<reference path="../node_modules/angular2/typings/browser.d.ts"/>

import {bootstrap} from 'angular2/platform/browser';
import {Component} from 'angular2/core';
import {Http, Response} from "angular2/http";
import 'rxjs/add/operator/map';
import {HTTP_PROVIDERS} from "angular2/http";
import {RemoveSpaces} from "./removespace.ts";


@Component({
	selector: 'proposalview',
	pipes: [RemoveSpaces],
	template: `

		<div class="box">
		<div class="header-container">
		<div class="logo">
			<img src="img/safex-logo.png">
		</div>
	<h1 class="main-head">Safe Exchange | Proposal View</h1>
		</div>

	    <div class="outer-box">

	    	<a href="/"><button class="small-btn first">Home</button></a>
		<a href="/submitproposal"><button class="small-btn">Submit Proposal</button></a>
		<br><p class="attribute">Title:</p> 
			{{ title }}
		<br><p class="attribute">The Terms:</p>
			{{ the_terms }}
		<br><p class="attribute">Responses:</p>

		<ul clas="list-unstyled">
			<li *ngFor="let resp of responses">
					{{ resp }}
			</li>
		</ul>

	<br><p class="attribute">Origin Public Key:</p>
			{{ origin_pubkey }}
		<br><div id="container"></div>
		<br><a href="/voteproposal/{{ nospace_title }}"><button class="small-btn">Vote on this Proposal</button></a>
		<br>
		<br>
		<h2>Results</h2>
		<ul>
				<li *ngFor="let response of result_responses">
					{{ response }} :
				</li>
		</ul>
		<ul>
				<li *ngFor="let result of result_results">
					{{ result }}
				</li>
		</ul>	
		
   	
	</div>
	</div>	



	`,
	directives: [],
	styleUrls: ['css/viewprop/css/bootstrap.css', 'css/viewprop/css/style.css']
})

export class ViewProposalComponent {
	title: string;
	nospace_title: string;
	the_terms: string;
	responses: string[] = [];
	origin_pubkey: string;
	hash: string;

	public result_responses;
	public result_results;

	public proposal;

    constructor(private _http: Http) {}

	open_proposal(body) {
		return this._http.post('http://localhost:3100/return_proposal', body)
			.map(res => res.json())
	}


	return_results(body) {
		return this._http.post('http://localhost:3100/return_results', body)
			.map(res => res.json())
	}


	loadProposal(stringified) {
		var self = this;
		this.open_proposal(stringified)
			.subscribe(
				data => { 
					self.proposal = data;
					self.title = data.title;
					self.the_terms = data.the_terms;
					self.responses = data.responses;
					self.origin_pubkey = data.origin_pubkey;
					self.hash = data.poll_hash;	
					console.log(self.proposal);
					var contents = JSON.stringify(data);
					var dater = "text/json;charset=utf-8," + encodeURIComponent(contents);
					var link = document.getElementById("container");
					link.innerHTML = '<a href="data:' + dater + '" download="proposal.poll">Download Proposal File</a>';
				},
				error => console.log("error"),
				() => console.log("finished")
			);
	}

	loadResults(stringified) {
		var self = this;
		this.return_results(stringified)
			.subscribe(
				data => {
					self.result_responses = data.responses;
					self.result_results = data.tally;
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
		this.loadResults(stringified);
		this.nospace_title = res[1];
	}




}

bootstrap(ViewProposalComponent, [HTTP_PROVIDERS]);
