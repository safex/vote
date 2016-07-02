import {Injectable} from "angular2/core";
import {Http} from "angular2/http";
import 'rxjs/add/operator/map';

@Injectable()

export class ProposalService {
	constructor(private _http: Http) {}

	get_key() {
		return this._http.get('http://localhost:3000/getpub')
			.map(res => res.json())
	}

	set_key(body) {
		return this._http.post('http://localhost:3000/setkey', body)
			.map(res => res.json())
	}

	make_proposal(body) {
		return this._http.post('http://localhost:3000/makeproposal', body)
			.map(res => res.json())
	}

	get_proposal() {
		return this._http.get('http://localhost:3000/getproposal')
			.map(res => res.json())
	}

}