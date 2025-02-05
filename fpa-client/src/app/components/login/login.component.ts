import { ChangeDetectionStrategy, Component } from '@angular/core';
import { FormControl, FormGroup, ReactiveFormsModule, Validators } from '@angular/forms';
import { MatCardModule } from '@angular/material/card';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatIconModule } from '@angular/material/icon';
import { MatButtonModule } from '@angular/material/button';
import { MatDividerModule } from '@angular/material/divider';
import { AuthService } from '../../services/auth.service';
import { Router } from '@angular/router';

@Component({
	selector: 'app-login',
	imports: [ReactiveFormsModule, MatCardModule, MatFormFieldModule, MatInputModule, MatIconModule, MatButtonModule, MatDividerModule],
	templateUrl: './login.component.html',
	styleUrl: './login.component.scss',
	changeDetection: ChangeDetectionStrategy.OnPush,
})
export class LoginComponent {

	hide: boolean = true;

	form = new FormGroup({
		username: new FormControl('', Validators.required),
		password: new FormControl('', Validators.required)
	});

	constructor(
		private auths: AuthService,
		private router: Router
	) { }

	submit() {
		this.auths.logout();
		this.auths.login(this.form.value).subscribe({
			next: (v) => { },
			error: (e) => {
				console.error(e);
				// TODO Notificar falha de login.
				window.alert('Não foi possível autenticar o usuário.');
			},
			complete: () => {
				this.router.navigate(['/home']).then(_ => console.info("Usuário autenticado"));
			}
		})
	}

	google() {
		throw new Error('Method not implemented.');
	}

	linkedin() {
		throw new Error('Method not implemented.');
	}

	signup() {
		throw new Error('Method not implemented.');
	}
}
