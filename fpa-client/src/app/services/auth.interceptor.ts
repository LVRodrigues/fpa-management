import type { HttpInterceptorFn } from '@angular/common/http';
import { inject } from '@angular/core';
import { TokenService } from './token.service';

export const authInterceptor: HttpInterceptorFn = (req, next) => {
	const tokens = inject(TokenService);
	const token = tokens.getToken();

	let modified = req;

	if (token) {
		modified = req.clone({
			setHeaders: {
				Authorization: 'Bearer ' + token
			}
		});
	}

	if (!req.headers.has('Content-Type')) {
		modified = modified.clone({
			setHeaders: {
				'Content-Type': 'application/json',
			}
		});
	}

	if (!req.headers.has('Accept')) {
		modified = modified.clone({
			setHeaders: {
				'Accept': 'application/json'
			}
		});
	}

	return next(modified);
};
