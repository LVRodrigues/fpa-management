import { Routes } from '@angular/router';
import { HomeComponent } from './components/home/home.component';

export const routes: Routes = [
    { path: '', redirectTo: 'home', pathMatch: 'full' },
    // { path: 'home', canActivate: [ authGuard ], component: HomeComponent },
    { path: 'home', component: HomeComponent },
    { path: '**', redirectTo: 'home'}
];
