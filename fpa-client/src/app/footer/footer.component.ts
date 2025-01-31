import { ChangeDetectionStrategy, Component } from '@angular/core';
import { environment } from '../../environments/environment';

@Component({
    selector: 'app-footer',
    imports: [],
    templateUrl: './footer.component.html',
    styleUrl: './footer.component.css',
    changeDetection: ChangeDetectionStrategy.OnPush,
})
export class FooterComponent {

    version = environment.version;
    release = environment.release;

}
