import { ChangeDetectionStrategy, Component } from '@angular/core';
import { MatToolbarModule } from '@angular/material/toolbar';
import { AppComponent } from '../../app.component';

@Component({
	selector: 'app-header',
	imports: [MatToolbarModule],
	templateUrl: './header.component.html',
	styleUrl: './header.component.scss',
	changeDetection: ChangeDetectionStrategy.OnPush,
})
export class HeaderComponent {

	title: String = "";

	constructor(private app: AppComponent) {
		this.title = app.title;
	 }

}
