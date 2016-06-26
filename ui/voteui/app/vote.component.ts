import {Component} from 'angular2/core';
import {VoteService} from "./vote.service";


@Component({
	selector: 'vote',
	template: `
		<div class="header">
	<h3 class="main-head">Voting Application</h3>
	    <div class="head-strip">

		<p class="attribute">Your Public Key: <span class="text-value">{{pubKey}}</span></p>
		
		<input class="input-field" placeholder="Input your Wallet Import Format (WIF) private key here" #importbox><button class="small-btn" (click)="postWIF(importbox)">Import Key</button><br>

		<p class="attribute"> Upload the proposal you wish to vote on </p>
		<input type="file" (change)="changeListener($event)" #input/>

		<p class="attribute">Proposal Title: <span class="text-value">{{ title }}</span></p>
		<p class="attribute">Proposal Terms: <span class="text-value">{{ the_terms }}</span></p>

		<p class="attribute">Responses are:</p> 
		<ul>
			<li class="answers" *ngFor="let response of responses; let i=index">
				 {{ response }} <button class="small-btn" (click)="makeSelection(i)">Select {{ i }}</button>
			</li>
		</ul>

		<p class="attribute">Vote Selection: <span class="text-value">{{ selection }}</span></p> 

		<button class="small-btn" (click)="makeVote()">Generate Vote</button>

		<button class="small-btn" (click)="saveVote()">Save Vote To Desktop</button>

		<br><br><div id="container"></div>
	</div>
	</div>
	`,
	providers: [VoteService],
	styleUrls: ['css/bootstrap.css', 'css/style.css']
})


export class VoteComponent {
	pubKey: string;
	title: string;
	the_terms: string;
	responses: string[];
	selection: string;
	vote_data: string;


    constructor(private _voteService: VoteService) {}

	changeListener(event) {
		var reader = new FileReader();
		var contents = "";
		var self = this;
        reader.onload = function(e:any) {
			contents = JSON.parse(e.target.result);
			var title = "title";
			var the_terms = "the_terms";
			var responses = "responses";
			self.title = contents[title];
       		self.the_terms = contents[the_terms];
       		self.responses = contents[responses];
       		self._voteService.set_proposal(JSON.stringify(contents))
       			.subscribe(
       				data => console.log("sent"),
       				error => console.log("error getting data here its fine"),
       				() => console.log("finished subscribe")
       			);
        };
        // read the image file as a data URL.
        reader.readAsText(event.target.files[0]);
    }

    makeVote() {
    	var vote_index = "vote_index";
		var json = {};
		json[vote_index] = +this.selection;
		this._voteService.set_vote(JSON.stringify(json))
		.subscribe(
			data => console.log("success"),
			error => console.log("error getting data here its fine"),
			() => console.log("finished")
		);
    }

    saveVote() {
    	var contents;
    	this._voteService.get_vote()
		.subscribe(
			data => {
				contents = JSON.stringify(data);
				var dater = "text/json;charset=utf-8," + encodeURIComponent(contents);
				var link = document.getElementById("container");
				link.innerHTML = '<a href="data:' + dater + '" download="vote.vote">Save Vote File</a>';
			},
			error => console.log("error getting data here its fine"),
			() => console.log("finished")
		);

    }

    makeSelection(i) {
    	console.log(i);
    	this.selection = i;
    }


    getKey() {
    	this._voteService.get_key()
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
		this._voteService.set_key(JSON.stringify(json))
			.subscribe(
				data => console.log("finished import"),
       			error => console.log("error getting data here its fine"),
				() => console.log("finished import")
			);
		this.getKey();
    }

}