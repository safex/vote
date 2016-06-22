import {Component} from "@angular/core";
import {ImportService} from "./import.service";

@Component({
	selector: 'import',
	template: `
		<input placeholder="Input your Wallet Import Format (WIF) private key here" #importbox><button (click)="import_key(importbox)">Import</button><br>
		<button (click)="refresh_app()">Refresh</button>
		<p>pubkey: {{pubKey}}</p>
	`,
	providers: [ImportService]
})

export class ImportComponent {
	pubKey: string;

	constructor(private _importService: ImportService) {}

	import_key(importbox) {
		var key = "wif";
		var json = {};
		json[key] = importbox.value;
		console.log(importbox);
		this._importService.set_key(JSON.stringify(json))
			.subscribe(
				data => console.log("finished import"),
				error => this.pubKey = "problem with import",
				() => console.log("finished import")
			);
	}

	refresh_app() {
		this._importService.get_key()
			.subscribe(
				data => this.pubKey = JSON.stringify(data),
				error => this.pubKey = "Error",
				() => console.log("finished")
			);

	}


}
