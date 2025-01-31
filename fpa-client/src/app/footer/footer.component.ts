import { ChangeDetectionStrategy, Component } from '@angular/core';
import { environment } from '../../environments/environment';
import { CommonModule } from '@angular/common';

@Component({
    selector: 'app-footer',
    imports: [CommonModule],
    templateUrl: './footer.component.html',
    styleUrl: './footer.component.css',
    changeDetection: ChangeDetectionStrategy.OnPush,
})
export class FooterComponent {

    version = environment.version;
    release = environment.release;

}
