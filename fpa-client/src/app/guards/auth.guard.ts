import { inject } from '@angular/core';
import { CanActivateFn, Router } from '@angular/router';
import { AuthService } from '../services/auth.service';
import { TokenService } from '../services/token.service';

export const authGuard: CanActivateFn = (route, state) => {
	const auths = inject(AuthService);
	const tokens = inject(TokenService);
	const router = inject(Router);
	const url: string = state.url;

	if (tokens.getRefreshToken()) {
		return true;
	}

	console.log("Redirection to login...");
	auths.redirectUrl = url;
	router.navigate(['/login']).then(_ => false);
	return false;
};
