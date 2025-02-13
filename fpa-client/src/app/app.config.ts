import { ApplicationConfig, provideZoneChangeDetection } from '@angular/core';
import { provideRouter } from '@angular/router';

import { routes } from './app.routes';
import { provideAnimationsAsync } from '@angular/platform-browser/animations/async';
import { provideHttpClient } from '@angular/common/http';

import {
	provideKeycloak,
	withAutoRefreshToken,
	AutoRefreshTokenService,
	UserActivityService,
	createInterceptorCondition,
	IncludeBearerTokenCondition,
	INCLUDE_BEARER_TOKEN_INTERCEPTOR_CONFIG
} from 'keycloak-angular';
import { environment } from '../environments/environment';

const urlCondition = createInterceptorCondition<IncludeBearerTokenCondition>({
	urlPattern: /^(http:\/\/localhost:(8080|5000))(\/.*)?$/i,
	bearerPrefix: 'Bearer'
});

export const provideKeycloakAngular = () =>
	provideKeycloak({
		config: {
			url: environment.keycloak.url,
			realm: environment.keycloak.realm,
			clientId: environment.keycloak.clientId
		},
		initOptions: {
			onLoad: 'login-required',
			checkLoginIframe: false
		},
		features: [
			withAutoRefreshToken({
				onInactivityTimeout: 'logout',
				sessionTimeout: environment.keycloak.sessionTimeout
			})
		],
		providers: [
			AutoRefreshTokenService,
			UserActivityService,
			{
				provide: INCLUDE_BEARER_TOKEN_INTERCEPTOR_CONFIG,
				useValue: [urlCondition]
			}
		]
	});


export const appConfig: ApplicationConfig = {
	providers: [
		provideZoneChangeDetection({ eventCoalescing: true }),
		provideRouter(routes),
		provideAnimationsAsync(),
		provideHttpClient(),
		provideKeycloakAngular(),
	]
};
