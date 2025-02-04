import { HttpClient, HttpHeaders, HttpParams } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { TokenService } from './token.service';
import { Router } from '@angular/router';

import { environment } from '../../environments/environment';
import { Observable, tap } from 'rxjs';

const OAUTH_CLIENT = environment.oauthClientId;
const OAUTH_SECRET = environment.oauthSecret;
const API_URL = environment.loginUrl;

const HTTP_OPTIONS = {
	headers: new HttpHeaders({
		'Content-Type': 'application/x-www-form-urlencoded'
	})
};

@Injectable({
	providedIn: 'root'
})
export class AuthService {

	redirectUrl: string;

	constructor(
		private http: HttpClient,
		private tokens: TokenService,
		private router: Router
	) {
		this.redirectUrl = '';
	}

	login(data: any): Observable<any> {
		this.logout();
		const body = new HttpParams()
			.set('grant_type', 'password')
			.set('client_id', OAUTH_CLIENT)
			.set('client_secret', OAUTH_SECRET)
			.set('username', data.username)
			.set('password', data.password);

		return this.http.post<any>(API_URL, body, HTTP_OPTIONS)
			.pipe(
				tap(res => {
					this.tokens.saveToken(res.access_token);
					this.tokens.saveRefreshToken(res.refresh_token);
				}),
			);
	}

	refreshToken(refreshToken: string): Observable<any> {
		const body = new HttpParams()
			.set('client_id', OAUTH_CLIENT)
			.set('client_secret', OAUTH_SECRET)
			.set('refresh_token', refreshToken)
			.set('grant_type', 'refresh_token');
		return this.http.post<any>(API_URL, body, HTTP_OPTIONS)
			.pipe(
				tap(res => {
					this.tokens.saveToken(res.access_token);
					this.tokens.saveRefreshToken(res.refresh_token);
				}),
			);
	}

	logout(): void {
		this.tokens.removeToken();
		this.tokens.removeRefreshToken();
		this.router.navigate(['/login']);
	}

	isLogged(): boolean {
		return (this.tokens.getToken() !== null);
	}
}
