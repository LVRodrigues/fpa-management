import { Routes } from '@angular/router';
import { HomeComponent } from './components/home/home.component';
import { ForbiddenComponent } from './components/forbidden/forbidden.component';
import { canActivateAuthRole } from './guards/auth-role.guard';

export const routes: Routes = [
    { path: '', redirectTo: 'home', pathMatch: 'full' },
    { path: 'home', canActivate: [ canActivateAuthRole ], component: HomeComponent, data: {role: 'user'} },
    { path: 'forbidden', component: ForbiddenComponent },
    { path: '**', redirectTo: 'home'}
];
