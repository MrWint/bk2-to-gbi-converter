//
//   Copyright (C) 2007 by sinamas <sinamas at users.sourceforge.net>
//
//   This program is free software; you can redistribute it and/or modify
//   it under the terms of the GNU General Public License version 2 as
//   published by the Free Software Foundation.
//
//   This program is distributed in the hope that it will be useful,
//   but WITHOUT ANY WARRANTY; without even the implied warranty of
//   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//   GNU General Public License version 2 for more details.
//
//   You should have received a copy of the GNU General Public License
//   version 2 along with this program; if not, write to the
//   Free Software Foundation, Inc.,
//   51 Franklin St, Fifth Floor, Boston, MA  02110-1301, USA.
//

#include "cinterface.h"
#include "gambatte.h"
#include <cstdlib>
#include <cstring>

// new is actually called in a few different places, so replace all of them for determinism guarantees
void *operator new(std::size_t n) {
	void *p = std::malloc(n);
	std::memset(p, 0, n);
	return p;
}

void operator delete(void *p) {
	std::free(p);
}

namespace {

using namespace gambatte;

	GBEXPORT GB * gambatte_create() {
	return new GB();
}

GBEXPORT void gambatte_destroy(GB *g) {
	delete g;
}

GBEXPORT int gambatte_load(GB *g, char const *romfiledata, unsigned romfilelength, unsigned flags) {
	return g->load(romfiledata, romfilelength, flags);
}

GBEXPORT int gambatte_loadbios(GB *g, char const *biosfiledata, unsigned size) {
	return g->loadBios(biosfiledata, size);
}

GBEXPORT int gambatte_runfor(GB *g, unsigned *samples) {
	std::size_t sampv = *samples;
	int ret = g->runFor(sampv);
	*samples = sampv;
	return ret;
}

GBEXPORT void gambatte_setvideobuffer(GB *g, uint_least32_t *const videoBuf, const int pitch) {
	g->setVideoBuffer(videoBuf, pitch);
}

GBEXPORT void gambatte_setrtcdivisoroffset(GB *g, int rtcDivisorOffset) {
	g->setRtcDivisorOffset(rtcDivisorOffset);
}

GBEXPORT void gambatte_setinputgetter(GB *g, unsigned (*getinput)(void *, unsigned), void *context) {
	g->setInputGetter(getinput, context);
}

}
