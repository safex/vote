///<reference path="../node_modules/angular2/typings/browser.d.ts"/>

import {bootstrap} from 'angular2/platform/browser';
import {Component} from 'angular2/core';
import {Http, Response} from "angular2/http";
import 'rxjs/add/operator/map';
import {HTTP_PROVIDERS} from "angular2/http";
import {RemoveSpaces} from "./removespace.ts";

@Component({
	selector: 'app',
	pipes: [RemoveSpaces],
	template: `
		<div class="box">
		<div class="header-container">
		<div class="logo">
			<img src="img/safex-logo.png">
		</div>
	<h1 class="main-head">Safe Exchange Community Voting</h1>
		</div>

	    <div class="outer-box">

	    	<a href="/"><button class="small-btn first">Home</button></a>
		<a href="/submitproposal"><button class="small-btn">Submit Proposal</button></a>
		<ul clas="list-unstyled">
			<li *ngFor="let proposal of proposals">
				<br>{{ proposal.title | json }} 
				<br>{{ proposal.hash | json }} 
				<br>
				<a href="/viewproposal/{{proposal.hash}}{{proposal.title | removeSpaces}}"><button class="small-btn first">View Proposal</button></a>
				<a href="/voteproposal/{{proposal.hash}}{{proposal.title | removeSpaces}}"><button class="small-btn">Vote on this Proposal</button></a>
			</li>
		</ul>
   	
	</div>
	</div>	

	`,
	directives: [],
	styleUrls: ['css/home/css/bootstrap.css', 'css/home/css/style.css']
})

export class AppComponent {
	public proposals;

    constructor(private _http: Http) {}

   	get_proposals() {
		return this._http.get('http://localhost:3100/return_proposals')
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
