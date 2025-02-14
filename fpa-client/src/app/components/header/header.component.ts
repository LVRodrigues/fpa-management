import { ChangeDetectionStrategy, Component } from '@angular/core';
import { MatToolbarModule } from '@angular/material/toolbar';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatDividerModule } from '@angular/material/divider';
import { AppComponent } from '../../app.component';
import Keycloak from 'keycloak-js';

@Component({
	selector: 'app-header',
	imports: [MatToolbarModule, MatButtonModule, MatIconModule, MatDividerModule],
	templateUrl: './header.component.html',
	styleUrl: './header.component.scss',
	changeDetection: ChangeDetectionStrategy.OnPush,
})
export class HeaderComponent {

	username: String = "";
	title: String | undefined = "";

	constructor(
		private app: AppComponent,
		private keycloak: Keycloak
	) {
		this.title = app.title;
		this.username = keycloak.tokenParsed?.['name'];
	}

	signout(): void {
		this.keycloak.logout();
	}
}
