import {Injectable} from "@angular/core";
import {Http} from "@angular/http";
import 'rxjs/add/operator/map';

@Injectable() 

export class ImportService {
	constructor(private _http: Http) {}

	get_key() {
		return this._http.get('http://localhost:3000/getpub')
			.map(res => res.json())
	}

	set_key(body) {
		return this._http.post('http://localhost:3000/setkey', body).map(res => res.json())
	}
}