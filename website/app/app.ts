///<reference path="../node_modules/angular2/typings/browser.d.ts"/>

import {bootstrap} from 'angular2/platform/browser';
import {Component} from 'angular2/core';
import {Http, Response} from "angular2/http";
import 'rxjs/add/operator/map';
import {HTTP_PROVIDERS} from "angular2/http";

@Component({
	selector: 'app',
	template: `
		<h1>Safe Exchange Community Voting</h1>
		<a href="/">Home</a>
		<a href="/submitproposal">Submit Proposal</a>

		<ul>
			<li *ngFor="let proposal of proposals">
				 {{ proposal.title | json }} 
				 {{ proposal.hash | json }} 
				 <button>View Details</button>
			</li>
		</ul>

	`,
	directives: []
})

export class AppComponent {
	public proposals;

    constructor(private _http: Http) {}

   	get_proposals() {
		return this._http.get('http://localhost:3100/return_proposals')
			.map(res => res.json())
	}

	open_proposal(body) {
		return this._http.get('http://localhost:3100/return_proposal', body)
			.map(res => res.json())
	}

	readproposals() {

		this.get_proposals()
			.subscribe( 
				data => {
					this.proposals = data;
				},
				error => console.log("error"),
				() => console.log("finished")
			);
	}

	ngOnInit() {
		this.readproposals();
	}

}

bootstrap(AppComponent, [HTTP_PROVIDERS]);
