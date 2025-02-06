import { Routes } from '@angular/router';
import { authGuard } from './services/auth.guard';
import { LoginComponent } from './components/login/login.component';
import { HomeComponent } from './components/home/home.component';

export const routes: Routes = [
    { path: '', redirectTo: 'home', pathMatch: 'full' },
    { path: 'home', canActivate: [ authGuard ], component: HomeComponent },
    { path: 'login', component: LoginComponent },
    { path: '**', redirectTo: 'home'}
];
