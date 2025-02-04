import { Injectable } from '@angular/core';
import { JwtHelperService } from '@auth0/angular-jwt';

const ACCESS_TOKEN = 'access_token';
const REFRESH_TOKEN = 'refresh_token';
const SUBJECT = "subject";

@Injectable({
    providedIn: 'root'
})
export class TokenService {

    private helper: JwtHelperService;

    constructor() {
        this.helper = new JwtHelperService();
    }

    getToken(): string | null {
        return localStorage.getItem(ACCESS_TOKEN);
    }

    getRefreshToken(): string | null {
        return localStorage.getItem(REFRESH_TOKEN);
    }

    saveToken(token: string): void {
        localStorage.setItem(ACCESS_TOKEN, token);
    }

    saveRefreshToken(refreshToken: string): void {
        localStorage.setItem(REFRESH_TOKEN, refreshToken);
    }

    removeToken(): void {
        localStorage.removeItem(ACCESS_TOKEN);
    }

    removeRefreshToken(): void {
        localStorage.removeItem(REFRESH_TOKEN);
    }

    getName(): string {
        let token: any = this.getToken();
        let decoded = this.helper.decodeToken(token);
        return decoded.name;
    }
}
