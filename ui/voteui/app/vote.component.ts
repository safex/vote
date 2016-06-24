import {Component} from 'angular2/core';
import {VoteService} from "./vote.service";


@Component({
	selector: 'vote',
	template: `
		<button (click)="getKey()">test GET request</button><br>
		<p>Output: {{pubKey}}</p>
		
		<input placeholder="Input your Wallet Import Format (WIF) private key here" #importbox><button (click)="postWIF(importbox)">Import Key</button><br>

		<p> Upload the proposal you wish to vote on </p>
		<input type="file" (change)="changeListener($event)" #input/>

		<p>Proposal Title: {{ title }}</p>
		<p>Proposal Terms: {{ the_terms }}</p>
		Responses are: <ul>
			<li *ngFor="let response of responses">
				 {{ response }}
			</li>
		</ul>
	`,
	providers: [VoteService]
})


export class VoteComponent {
	pubKey: string;
	title: string;
	the_terms: string;
	responses: string[];


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
        };
        // read the image file as a data URL.
        reader.readAsText(event.target.files[0]);
    }

    constructor(private _voteService: VoteService) {}

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
				error => console.log("no data received"),
				() => console.log("finished import")
			);
    }

}