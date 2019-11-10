// This file is part of the Orbbec Astra SDK [https://orbbec3d.com]
// Copyright (c) 2015-2017 Orbbec 3D
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Be excellent to each other.
#ifndef _ASTRA_DEIVCE_CONTROL_H_
#define _ASTRA_DEIVCE_CONTROL_H_

#include <astra_core/capi/astra_defines.h>
#include <astra_core/capi/astra_types.h>

ASTRA_BEGIN_DECLS

typedef struct {    
    const char *uri;
    const char *command;
    char *out;
    int32_t outLength;
    astra_status_t result;
} astra_device_command_t;

ASTRA_API_EX astra_status_t astra_distortion_enable(astra_streamsetconnection_t connect,int status);








ASTRA_END_DECLS

#endif