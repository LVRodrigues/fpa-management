import { Component } from '@angular/core';
import { MatToolbarModule } from '@angular/material/toolbar';
import { environment } from '../../../environments/environment';
import { CommonModule } from '@angular/common';

@Component({
	selector: 'app-footer',
	imports: [MatToolbarModule, CommonModule],
	templateUrl: './footer.component.html',
	styleUrl: './footer.component.scss'
})
export class FooterComponent {

	version = environment.version;
	release = environment.release;
}
