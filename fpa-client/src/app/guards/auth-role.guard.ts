import { AuthGuardData, createAuthGuard } from 'keycloak-angular';
import { ActivatedRouteSnapshot, CanActivateFn, Router, RouterStateSnapshot, UrlTree } from '@angular/router';
import { inject } from '@angular/core';

const isAccessAllowed = async (
	route: ActivatedRouteSnapshot,
	__: RouterStateSnapshot,
	authData: AuthGuardData
): Promise<boolean | UrlTree> => {
	const router = inject(Router);

	const { authenticated, grantedRoles } = authData;

	const requiredRole = route.data['role'];
	if (!requiredRole) {
		return true;
	}

	const hasRequiredRole = (role: string): boolean =>
		Object.values(grantedRoles.resourceRoles).some((roles) => roles.includes(role)) ||
		grantedRoles.realmRoles.includes(role);

	if (authenticated && hasRequiredRole(requiredRole)) {
		return true;
	}

	return router.parseUrl('/forbidden');
};

export const canActivateAuthRole = createAuthGuard<CanActivateFn>(isAccessAllowed);
