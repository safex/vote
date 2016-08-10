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
		<h1>Safe Exchange | Proposal View</h1>
		<a href="/">Home</a>
		<a href="/submitproposal">Submit Proposal</a>

		<br><b>Title:</b> 
			<br>{{ title }}
		<br><b>The Terms:</b>
			<br>{{ the_terms }}
		<br><b>Responses:</b>
			<ul>
				<li *ngFor="let resp of responses">
					{{ resp }}
				</li>
			</ul>
		<br><b>Origin Public Key:</b>
			<br>{{ origin_pubkey }}

		<br><button>Download Proposal File for Voting</button>

		<br><a href="/voteproposal/{{ nospace_title }}"><button>Vote on this Proposal</button></a>
		<br>
		<br>
		<h2> Results </h2>
		<ul>
				<li *ngFor="let response of result_responses; let result of result_results">
					{{ response }} : {{ result }}
				</li>
		</ul>

	`,
	directives: [],
	styleUrls: []
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
