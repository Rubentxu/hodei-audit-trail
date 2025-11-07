import { NextResponse } from 'next/server';
import { signOut } from 'next-auth/react';

export async function POST(request: Request) {
  try {
    // Get the request body if needed
    const body = await request.json().catch(() => ({}));
    const { callbackUrl = '/' } = body;

    // Create the sign-out URL
    const signOutUrl = new URL('/api/auth/signout', request.url);
    signOutUrl.searchParams.set('callbackUrl', callbackUrl);

    // Return the sign-out URL for the client to use
    return NextResponse.json({
      success: true,
      signOutUrl: signOutUrl.toString()
    });
  } catch (error) {
    console.error('Logout error:', error);
    return NextResponse.json(
      { success: false, error: 'Failed to initiate logout' },
      { status: 500 }
    );
  }
}
