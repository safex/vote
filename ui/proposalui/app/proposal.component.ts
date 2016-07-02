import {Component} from 'angular2/core';
import {ProposalService} from "./proposal.service";


@Component({
	selector: 'propose',
	template: `
		<div class="header">
		<h3 class="main-head">Proposal Application</h3>
	    <div class="head-strip">

		<p class="attribute">Your Public Key: <span class="text-value">{{pubKey}}</span></p>
		
		<input class="input-field" placeholder="Input your Wallet Import Format (WIF) private key here" #importbox><button class="small-btn" (click)="postWIF(importbox)">Import Key</button><br>

		<br><br><input placeholder="Enter the title for the proposal"><br>

		<br><br><textarea placeholder="Enter the terms of the proposal"></textarea><br>

		<br><br><input placeholder="Here enter a choice for people to select in voting"><button>Add Choice</button><br>

		<br><br><button>Create Proposal</button>

		<br><br><div id="container"></div>


	</div>
	</div>
	`,
	providers: [ProposalService],
	styleUrls: ['css/bootstrap.css', 'css/style.css']
})


export class ProposalComponent {
	pubKey: string;
	title: string;
	the_terms: string;
	choices: string[];


    constructor(private _proposalService: ProposalService) {}

	

    getKey() {
    	this._proposalService.get_key()
		.subscribe(
			data => this.pubKey = JSON.stringify(data),
			error => this.pubKey = "Error",
			() => console.log("finished")
		);
    }

    postWIF(importbox) {
    	var key = "wif";
		var json = {};
		json[key] = importbox.value;
		console.log(importbox);
		this._proposalService.set_key(JSON.stringify(json))
			.subscribe(
				data => console.log("finished import"),
       			error => console.log("error getting data here its fine"),
				() => console.log("finished import")
			);
		this.getKey();
    }

}